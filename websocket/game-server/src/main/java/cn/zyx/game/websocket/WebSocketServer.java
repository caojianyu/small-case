package cn.zyx.game.websocket;

import cn.zyx.game.entity.Role;
import com.alibaba.fastjson.JSONObject;
import org.springframework.stereotype.Component;

import javax.websocket.OnClose;
import javax.websocket.OnMessage;
import javax.websocket.OnOpen;
import javax.websocket.Session;
import javax.websocket.server.PathParam;
import javax.websocket.server.ServerEndpoint;
import java.io.IOException;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;

/**
 * @author caojianyu
 * @version 1.0.0
 * @date 2022-07-12 15:33
 * @email jieni_cao@foxmail.com
 */

@Component
@ServerEndpoint(value = "/game/{account}")
public class WebSocketServer {

    private static final Map<String, Session> clients = new ConcurrentHashMap<>();
    private static final Map<String, Role> roles = new ConcurrentHashMap<>();

    @OnMessage
    public void onMessage(String message, Session session) throws IOException {
        JSONObject jsonObject = JSONObject.parseObject(message);
        Integer type = jsonObject.getInteger("type");
        switch (type) {
            case 1:
                System.out.println("心跳信息");
                session.getBasicRemote().sendText(message);
                break;
            case 2:
                broadcast(1, session, message);
                break;
            case 3:
                // 角色移动，更新缓存的位置信息
                Role role = roles.get(jsonObject.getString("account"));
                role.setTop(jsonObject.getString("top"));
                role.setLeft(jsonObject.getString("left"));
                role.setRotate(jsonObject.getString("rotate"));
            default:
                broadcast(2, session, message);
        }

    }

    private void broadcast(int type, Session session, String message) throws IOException {
        // 广播模式1全体广播，2除自己外全体广播
        for (Map.Entry<String, Session> m : clients.entrySet()) {
            if (type == 1) {
                m.getValue().getBasicRemote().sendText(message);
            } else {
                if (!m.getValue().getId().equals(session.getId())) {
                    m.getValue().getBasicRemote().sendText(message);
                }
            }
        }
    }

    @OnOpen
    public void onOpen(@PathParam("account") String account, Session session) throws IOException {
        Role role = new Role();
        role.setAccount(account);
        role.setLeft("0");
        role.setTop("20px");
        role.setRotate("rotate(0deg)");

        roles.put(account, role);

        clients.put(account, session);

        broadcastRoles();
        System.out.println("当前已有" + clients.size());
    }

    @OnClose
    public void onClose(Session session) throws IOException {
        String key = "";
        for (Map.Entry<String, Session> m : clients.entrySet()) {
            if (m.getValue().getId().equals(session.getId())) {
                key = m.getKey();
                break;
            }
        }

        // 删除连接
        clients.remove(key);
        // 删除缓存的角色信息
        roles.remove(key);

        Map<String, Object> map = new HashMap<>();
        map.put("type", 5);
        map.put("account", key);
        for (Map.Entry<String, Session> m : clients.entrySet()) {
            m.getValue().getBasicRemote().sendText(JSONObject.toJSONString(map));
        }

        System.out.println("退出一个用户");
        System.out.println("当前还有用户" + clients.size());
    }

    /**
     * 广播角色列表
     */
    private void broadcastRoles() throws IOException {
        Map<String, Object> map = new HashMap<>();
        map.put("type", 4);
        List<Role> list = new ArrayList<>();
        for (Map.Entry<String, Role> m : roles.entrySet()) {
            list.add(m.getValue());
        }

        map.put("roles", list);

        for (Map.Entry<String, Session> m : clients.entrySet()) {
            m.getValue().getBasicRemote().sendText(JSONObject.toJSONString(map));
        }
    }

}
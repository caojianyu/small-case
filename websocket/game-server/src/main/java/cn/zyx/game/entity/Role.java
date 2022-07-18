package cn.zyx.game.entity;

import javax.websocket.Session;

/**
 * @author caojianyu
 * @version 1.0.0
 * @date 2022-07-18 23:12
 * @email jieni_cao@foxmail.com
 */
public class Role {

    private String account;
    private String top;
    private String left;
    private String rotate;

    public String getAccount() {
        return account;
    }

    public void setAccount(String account) {
        this.account = account;
    }

    public String getTop() {
        return top;
    }

    public void setTop(String top) {
        this.top = top;
    }

    public String getLeft() {
        return left;
    }

    public void setLeft(String left) {
        this.left = left;
    }

    public String getRotate() {
        return rotate;
    }

    public void setRotate(String rotate) {
        this.rotate = rotate;
    }

}

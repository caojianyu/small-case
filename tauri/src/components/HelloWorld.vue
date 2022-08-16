<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { fetch } from "@tauri-apps/api/http";
import { readBinaryFile, BaseDirectory } from "@tauri-apps/api/fs";
import { resourceDir, join } from "@tauri-apps/api/path";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import { ask } from "@tauri-apps/api/dialog";
import { confirm } from "@tauri-apps/api/dialog";
import { open } from "@tauri-apps/api/dialog";
import { appDir } from "@tauri-apps/api/path";
import { appWindow } from "@tauri-apps/api/window";

import { WebviewWindow } from "@tauri-apps/api/window";

defineProps<{ msg: string }>();

const count = ref(0);
const imgSrc = ref();
onMounted(() => {
  setTimeout(() => {
    invoke("close_splashscreen");
  }, 3000);
  // console.log(a * b);
});

const myCustomCommand = async () => {
  const a = 0.07;
  const b = 100;
  // Invoke the command
  let c = await invoke("my_custom_command", { a, b });
  console.log(c);
};

const initProcess = async () => {
  await invoke("init_process");
  await listen<string>("event-name", (event) => {
    console.log(event);
  });
};

const testHttp = async () => {
  let data = await fetch(
    "https://v0.yiketianqi.com/api?unescape=1&version=v91&appid=43656176&appsecret=I42og6Lm&ext=&cityid=&city=%E6%88%90%E9%83%BD",
    {
      method: "GET",
      timeout: 30,
    }
  );
  console.log(data);
};

const readImg = async () => {
  // Read the image file in the `$RESOURCEDIR/avatar.png` path
  const contents = await readBinaryFile("img/32x32.png", {
    dir: BaseDirectory.Resource,
  });

  console.log(contents);
};

const readImgTwo = async () => {
  const resourceDirPath = await resourceDir();
  const filePath = await join(resourceDirPath, "img\\32x32.png");
  const assetUrl = convertFileSrc(filePath);

  imgSrc.value = assetUrl;
};

const dialogOne = async () => {
  const yes = await ask("Are you sure?", "Tauri");
  console.log(yes);
};
const dialogTwo = async () => {
  const yes2 = await ask("This action cannot be reverted. Are you sure?", {
    title: "Tauri",
    type: "warning",
  });
  console.log(yes2);
};

const confirmOne = async () => {
  const confirmed = await confirm("Are you sure?", "Tauri");
  console.log(confirmed);
};

const confirmTwo = async () => {
  const confirmed2 = await confirm(
    "This action cannot be reverted. Are you sure?",
    { title: "Tauri", type: "warning" }
  );
  console.log(confirmed2);
};

const selectDir = async () => {
  // Open a selection dialog for directories
  const selected = await open({
    directory: true,
    multiple: false,
    defaultPath: await appDir(),
  });
  if (Array.isArray(selected)) {
    // user selected multiple directories
  } else if (selected === null) {
    // user cancelled the selection
  } else {
    // user selected a single directory
  }
  console.log(selected);
};

const minimize = () => {
  appWindow.minimize();
};
const maximize = () => {
  appWindow.toggleMaximize();
};
const close = () => {
  appWindow.hide();
  // appWindow.close();
};

const newWindow = () => {
  appWindow.hide();
  const webview = new WebviewWindow("theUniqueLabel", {
    url: "test.html",
  });
  // since the webview window is created asynchronously,
  // Tauri emits the `tauri://created` and `tauri://error` to notify you of the creation response
  webview.once("tauri://created", function () {
    // webview window successfully created
    console.log("create success");
    appWindow.close();
  });
  webview.once("tauri://error", function (e) {
    // an error happened creating the webview window
  });
};

const newWindowTwo = () => {
  const testWindow = WebviewWindow.getByLabel("test");
  testWindow?.show();
};
</script>

<template>
  <div data-tauri-drag-region class="titlebar">
    <div @click="minimize()" class="titlebar-button" id="titlebar-minimize">
      <img
        src="https://api.iconify.design/mdi:window-minimize.svg"
        alt="minimize"
      />
    </div>
    <div @click="maximize()" class="titlebar-button" id="titlebar-maximize">
      <img
        src="https://api.iconify.design/mdi:window-maximize.svg"
        alt="maximize"
      />
    </div>
    <div @click="close()" class="titlebar-button" id="titlebar-close">
      <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
    </div>
  </div>
  <div class="btn-group">
    <button @click="myCustomCommand">触发rust方法</button>
    <button @click="initProcess">启动事件</button>
    <button @click="testHttp">测试接口</button>
    <button @click="readImg">读取图片</button>
    <button @click="readImgTwo">读取图片2</button>
    <button @click="dialogOne">弹框1</button>
    <button @click="dialogTwo">弹框2</button>
    <button @click="confirmOne">弹框1</button>
    <button @click="confirmTwo">弹框2</button>
    <button @click="selectDir">选择文件夹</button>
    <button @click="newWindow">打开新窗口</button>
    <button @click="newWindowTwo">打开新窗口2</button>
  </div>
  <img :src="imgSrc" alt="" />
</template>

<style scoped>
.titlebar {
  height: 30px;
  background: #329ea3;
  user-select: none;
  display: flex;
  justify-content: flex-end;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
}
.titlebar-button {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 30px;
  height: 30px;
}
.titlebar-button:hover {
  background: #5bbec3;
}

.btn-group {
  margin-top: 30px;
}
</style>

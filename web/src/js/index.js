import { Icon } from './custom-elements/x-icon';
import { Module } from './module';
import { Command } from './editor';
import '../css/style.css';

customElements.define('x-icon', Icon);

Module.load('./monkey.wasm', {
  print: (value) => {
    const str = Module.copyCStr(value);
    console.log(str);
    Command.print(str);
  },
  sleep: (time) => {
    console.log(time)
    const stop = new Date().getTime();
    while (new Date().getTime() < stop + time) {
        ;
    }
  },
  clear: () => {
    Command.clear();
  },
  random: (input) => {
    return Math.floor(Math.random() * input)
  },
  request: (value) => {
    const str = Module.copyCStr(value);
    const xhr = new XMLHttpRequest();
    xhr.open("GET", str, false);
    xhr.send();
    if (xhr.status === 200) {
      console.log(xhr.responseText)
      return Module.allocStr(xhr.responseText).ptr;
    } else {
      throw new Error('Request failed: ' + xhr.statusText);
    }
  },
}).catch((e) => console.error(e));

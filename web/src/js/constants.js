export const SHARE_QUERY_KEY = 's';

export const SNIPPETS = [
  {
    label: 'Variable bindings',
    value: `给 岁月 以 "文明";

岁月
`,
  },
  {
    label: 'Plus',
    value: `给 自然选择 以 0;

自然选择 前进 4
`,
  },
  {
    label: 'Minus',
    value: `给 宇宙 以 { "维度": 10 };

宇宙["维度"] 降维 7
`,
  },
  {
    label: 'True',
    value: `return 这是计划的一部分`,
  },
  {
    label: 'False',
    value: `return 主不在乎`,
  },
  {
    label: 'Function',
    value: `给 黑暗森林 以 法则() {
    给 基本公理 以 ["生存是文明的第一需要", "文明不断增长和扩张，但宇宙中的物质总量保持不变"];
    基本公理
}

黑暗森林()
`,
  },
  {
    label: 'Loop',
    value: `给 面壁计划 以 法则() {
      给 危机纪元 以 3;
      给 人数 以 4;
      面壁 (危机纪元 < 400) {

          给 危机纪元 = 危机纪元 + 1;

          if (危机纪元 == 8) {
              给 人数 以 人数 - 1;
              延续;
          }
          if (危机纪元 == 23) {
              给 人数 以 人数 - 1;
              延续;
          }
          if (危机纪元 == 205) {
              给 人数 以 人数 - 1;
          }

          广播([危机纪元, 人数]);

          if (危机纪元 == 205) {
              破壁;
          }
      }
  }
  
  面壁计划()
`,
  },
  {
    label: 'Print',
    value: `给 三体世界坐标 以 \"半人马星系\";

广播(三体世界坐标);

广播(三体世界坐标);

广播(三体世界坐标);
`,
  },
  {
    label: 'Sleep',
    value: `冬眠(1000);

true
`,
  },
  {
    label: 'Clear',
    value: `二向箔清理();
`,
  },
];

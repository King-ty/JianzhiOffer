// function largestRectangleArea(heights: number[]): number {
//     heights.push(0);
//     let st: number[] = [-1], len = heights.length;
//     let ret = 0;
//     for (let i = 0; i < len; ++i) {
//         while (heights.at(st.at(-1) as number) as number > heights[i]) {
//             let height = heights[st.pop() as number];
//             let width = i - (st.at(-1) as number) - 1;
//             ret = Math.max(ret, height * width);
//         }
//         st.push(i);
//     }
//     return ret;
// };

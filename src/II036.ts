function evalRPN(tokens: string[]): number {
    let numSt: number[] = [];
    for (let token of tokens) {
        let temp = Number(token);
        if (!isNaN(temp)) {
            numSt.push(temp);
        } else {
            let y = numSt.pop() as number, x = numSt.pop() as number;
            switch (token) {
                case "+":
                    numSt.push(x + y);
                    break;
                case "-":
                    numSt.push(x - y);
                    break;
                case "*":
                    numSt.push(x * y);
                    break;
                case "/":
                    numSt.push((x - x % y) / y); // js整除比较坑
                    break;
            }
        }
    }
    return numSt.pop() as number;
};

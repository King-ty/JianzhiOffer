class MovingAverage {
    size: number;
    list: number[];
    sum: number;
    constructor(size: number) {
        this.size = size;
        this.list = [];
        this.sum = 0;
    }

    next(val: number): number {
        this.sum += val;
        this.list.push(val);
        if (this.list.length > this.size) {
            this.sum -= this.list.shift() as number;
        }
        return this.sum / this.list.length;
    }
}

/**
 * Your MovingAverage object will be instantiated and called as such:
 * var obj = new MovingAverage(size)
 * var param_1 = obj.next(val)
 */

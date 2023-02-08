class RandomizedSet {
    mp: Map<number, number>
    vec: number[]
    constructor() {
        this.mp = new Map();
        this.vec = [];
    }

    insert(val: number): boolean {
        if (this.mp.has(val)) return false;
        this.vec.push(val);
        this.mp.set(val, this.vec.length - 1);
        return true
    }

    remove(val: number): boolean {
        if (!this.mp.has(val)) return false;
        let index = this.mp.get(val) as number, tail = this.vec.length - 1;
        this.vec[index] = this.vec[tail];
        this.mp.set(this.vec[tail], index);
        this.mp.delete(val);
        this.vec.pop();
        return true;
    }

    getRandom(): number {
        let rd = Math.floor(Math.random() * this.vec.length);
        return this.vec[rd];
    }
}

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * var obj = new RandomizedSet()
 * var param_1 = obj.insert(val)
 * var param_2 = obj.remove(val)
 * var param_3 = obj.getRandom()
 */

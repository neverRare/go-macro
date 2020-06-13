# Go Macro

Rust syntax in Rust program is overrated, why not use Go? This crate attempts to bring Go syntax with prelude and macro. Take a peek on how hideous your code can be with this crate!

```rust
use go_macro::{go, prelude::*};

go! {
    type Averager struct {
        members go!([]uint),
        average float64,
    }
}
go! {
    func (averager *Averager) update() {
        let mut sum: uint = 0
        go! {
            for member := range(&averager.members) {
                sum += member
            }
        }
        // it would be float64(...) instead of this,
        // rust's macro is pretty restrictive
        let average = go!((float64)(sum)) / go!((float64)(go!(len(&averager.members))))
        averager.average = average
    }
}
go! {
    func (averager *Averager) add(member uint) {
        // ref deref is something you'll not see in Go code,
        // but heh, its a pretty hack for rust's macro and ownership
        averager.members = go!(append(*&averager.members, member))
        averager.update()
    }
}
go! {
    func new_averager() Averager {
        return Averager {
            members: go!([]uint {}),
            average: 0.0,
        }
    }
}
go! {
    func main() {
        let mut averager = new_averager()
        averager.add(10)
        averager.add(25)
        averager.add(32)
        println!("{}", averager.average)
    }
}
```

As much as how unserious this is, this is my personal project for learning `macro_rules` in Rust. I know proc macro can do better, but this is more fun. Of course, I won't publish this to crates.io :) this project can cause disgrace.

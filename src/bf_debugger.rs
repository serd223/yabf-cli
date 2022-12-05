use yabf::BfInstance;

pub struct BfDebugConfig {
    pub dump_padding: usize,
}

impl Default for BfDebugConfig {
    fn default() -> Self {
        Self {
            dump_padding: 5
        }
    }
}

pub struct BfDebugger<const MEMSIZE: usize> {
    pub bf: BfInstance<MEMSIZE>,
    pub cfg: BfDebugConfig
}

impl<const MEMSIZE: usize> Default for BfDebugger<MEMSIZE> {
    fn default() -> Self {
        Self {
            bf: Default::default(),
            cfg: Default::default()
        }
    }
}


impl<const MEMSIZE: usize> BfDebugger<MEMSIZE> {

    pub fn dump_mem(&self, slice_len: usize) -> String {
        if slice_len >= MEMSIZE { panic!("Slice length can't be bigger than memory size.") }
        if self.cfg.dump_padding < 5 { panic!("Dump padding can't be less than 5.") }
        // Wrote this on my phone so it's a bit scuffed lol
        let padding = self.cfg.dump_padding;
        let start: usize = match self.bf.mem_ptr <= slice_len {
            true => 0,
            false => self.bf.mem_ptr - slice_len / 2
        };
        let end: usize = match start == 0 {
            true => slice_len,
            false => self.bf.mem_ptr + slice_len / 2 + match slice_len % 2 == 0 {
                false => 1,
                true => 0
            }
        };
        let mut res = " ".to_string();
        for _ in 0..(slice_len - 1) * (padding + 1) + padding {
            res += "_";
        }
        res += "\n|";
        
        for i in start..end {
            let n = &self.bf.mem[i];
            if i == self.bf.mem_ptr {
                let p = padding - 2;
                res += format!("[{:^p$}]|", n).as_str();
            } else {
                res += format!("{:^padding$}|", n).as_str();
            }
        }
        res += "\n ";
        for _ in 0..(slice_len - 1) * (padding + 1) + padding {
            res += "-";
        }
        res
    }
}
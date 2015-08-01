extern crate ilog2;
extern crate num;

use std::fmt::Binary;
use self::ilog2::Future;

pub trait Bitalloc {
    fn clear_all(&mut self);
    fn fill_all(&mut self);
    fn alloc(&mut self, size: usize) -> isize;
}

impl<T: ilog2::Bitops + Binary> Bitalloc for [T] {
	fn clear_all(&mut self) {
		for item in self.iter_mut() {
			*item = T::zero();
		}
	}
	fn fill_all(&mut self) {
		for item in self.iter_mut() {
			*item = ilog2::bit_mask::<T>();
		}
	}
	fn alloc(&mut self, size: usize) -> isize {
	    let block_size = ilog2::bit_length::<T>() as usize;
	    let block_count = size / block_size;
	    let rem_count = size % block_size;
	    
	    let mut blocks_allocated = 0usize;
	    let mut rem_allocated = 0usize;
	    let mut blocks_needed = block_count;
	    let mut rem_needed = rem_count;
	    
	    enum State {
	        FindFirst,
	        FindMid,
	        FindLast,
	    };
	    
	    let mut found_index = 0usize;
	    let mut found_offset = 0usize;
	    let mut found = false;
	    let mut state = State::FindFirst;
	    let mut index = 0usize;
	    let limit = self.len();
	    loop {
	        if index == limit {
	            break;
	        }
	        let val = self[index];
	        
	        match state {
	            State::FindFirst => {
	                //reset
	                blocks_allocated = 0;
	                rem_allocated = 0;
	                blocks_needed = block_count;
	                rem_needed = rem_count;
	                found_index = 0;
	                found_offset = 0;
	                
	                if size <= block_size { // allocated region may inside an integer
	                    let mut possible = val;
	                    let mut shifted = 0usize;
	                    loop {
	                        let current_available = possible.leading_zeros() as usize;
	                        //println!("debug2 {} {} {} {:064b}", current_available, size, shifted, possible);
	                        if current_available >= size {
	                            found = true;
	                            found_index = index;
	                            found_offset = shifted;
	                            break;
	                        }
	                        possible = possible << current_available; //discard current;
	                        shifted += current_available;
	                        //println!("debug3 {} {} {} {:064b}", current_available, size, shifted, possible);
	                        let current_skip = possible.leading_ones() as usize;
	                        shifted += current_skip;
	                        if (shifted + size) > block_size {
	                            break;
	                        }
	                        possible = possible << current_skip;
	                        possible = possible | ilog2::bit_mask::<T>() >> (block_size - shifted);
	                        //println!("debug4 {} {} {} {:064b}", current_available, size, shifted, possible);
	                    }
	                    if found {
	                        break;
	                    }
	                }
	                //size exceeded block size or failed to allocated inside a block
	                //find zeros from lsb position	                
	                let current_available = val.trailing_zeros() as usize;
	                if current_available > 0 {
	                    //possible start location
                        found_index = index;
                        found_offset = block_size - current_available;
    	                if current_available > rem_needed {
    	                    assert!(blocks_needed > 0);
    	                    rem_needed = rem_needed + block_size - current_available;
    	                    blocks_needed -= 1;
    	                }
    	                rem_allocated += current_available;
    	                state = State::FindMid;
	                }
	            },
	            State::FindMid => {
	                if blocks_allocated == blocks_needed {
	                    state = State::FindLast;
	                    continue;
	                } else if val == T::zero() {
	                    blocks_allocated += 1;
	                }
	            },
	            State::FindLast => {
	                let current_available = val.leading_zeros() as usize;
	                rem_allocated += current_available;
	                if rem_allocated >= rem_needed {
	                    found = true;
	                    break;
	                }
	                state = State::FindFirst;
	                continue;
	            },
	        }
	        index += 1;
	    }
	    if found {
	        
	        let mut allocated = 0usize;
	        
            let mut first_mask = ilog2::bit_mask::<T>();
            first_mask = first_mask >> found_offset;
            allocated += block_size - found_offset;
            if allocated > size {
                let diff = allocated - size;
                first_mask = first_mask >> diff;
                first_mask = first_mask << diff;
                allocated = size;
            }
            let mut update_index = found_index;
            assert!(self[update_index] & first_mask == T::zero());
            println!("Allocate first at block {} [{},{}] : {:064b}", update_index, found_index, found_offset, self[update_index]);
            self[update_index] = self[update_index] | first_mask;
            println!("Allocate first at block {} [{},{}] : {:064b}", update_index, found_index, found_offset, self[update_index]);
            update_index += 1;
            
            let rem = size - allocated;
            let fill = rem / block_size;
            let last_fill = rem % block_size;
            
            for i in 0 .. fill {
                println!("Allocate mid at block {} [{},{}] : {:064b}", update_index + i, found_index, found_offset, self[update_index + i]);
                let mid_mask = ilog2::bit_mask::<T>();
                assert!(self[update_index + i] & mid_mask == T::zero());
                self[update_index + i] = self[update_index + i] | mid_mask;
                allocated += block_size;
                println!("Allocate mid at block {} [{},{}] : {:064b}", update_index + i, found_index, found_offset, self[update_index + i]);
            }
            if last_fill > 0 {
                println!("Allocate last at block {} [{},{}] : {:064b}", update_index + fill, found_index, found_offset, self[update_index + fill]);
                let mut last_mask = ilog2::bit_mask::<T>();
                let diff = block_size - last_fill;
                last_mask = last_mask >> diff;
                last_mask = last_mask << diff;
                assert!(self[update_index + fill] & last_mask == T::zero());
                self[update_index + fill] = self[update_index + fill] | last_mask;
                allocated += last_fill;
                println!("Allocate last at block {} [{},{}] : {:064b}", update_index + fill, found_index, found_offset, self[update_index + fill]);
            }
            assert!(allocated == size);
            
            ((found_index * block_size) + found_offset) as isize
	    } else {
	        -1isize	        
	    }
	}
}
use std::collections::HashMap;
use std::collections::hash_map::Entry;

#[allow(dead_code)]
struct Cacher<T,A,R>
    where T: Fn(&A) -> R,
          A: std::cmp::Eq + std::hash::Hash,
          R: std::clone::Clone,
{
	func: T,
	results: HashMap<A, R>,
}
#[allow(dead_code)]
impl<T, A, R> Cacher<T,A,R>
    where T: Fn(&A) -> R,
         A: std::cmp::Eq + std::hash::Hash + std::clone::Clone,
         R: std::clone::Clone,
{
    fn new(func: T) -> Cacher<T, A, R> {
        let results: HashMap<A,R> = HashMap::new();
        Cacher {func, results}
    }


    fn value(&mut self, arg: A) -> R {
		match self.results.entry(arg) {
			Entry::Occupied(entry) => entry.get().clone(),
			Entry::Vacant(entry) => {
				let ret: R = (self.func)(entry.key());
				entry.insert(ret.clone());
				ret
			}
		}
    }
}

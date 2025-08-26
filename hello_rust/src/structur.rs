#[derive(Debug)]
struct Tringle {
	cat1: f32,
	cat2: f32
}

impl Tringle {
	fn find_hyp(&self) -> f32 {
		(self.cat1 + self.cat2)
	}
	
	fn create_isc(cat: f32) -> Self {
		Self {
			cat1: cat,
			cat2: cat
		}
	}
}

fn main() {
	let tr1 = Tringle {
		cat1: 8.0,
		cat2: 12.0
	};
	
	let hyp1 = tr1.find_hyp();
	let hyp2 = Tringle::create_isc(8.0); 
}
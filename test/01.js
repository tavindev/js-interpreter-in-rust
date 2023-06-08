function makeCounter() {
	let i = 0;
	
	function count() {
		i = i + 1;
		print i; 
	}

	return count;
}

let counter = makeCounter();
counter(); 
counter();
print counter;
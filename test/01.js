let a = "global";
let a = "not global";
  {
    function showA() {
      print a;
		}

    showA();
    let a = "block";
    showA();
}
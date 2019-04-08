var days = ["Lundi", "Lundi", "Mardi", "Mercredi", "Jeudi", "Vendredi", "Vendredi"];
var d = new Date();
var current_day = days[d.getDay()];


function init() {
	document.getElementById(current_day).classList.add("active"); 

}

window.onload = init;


function getProjects() {
    fetch("https://gitlab.sokoll.com/api/v4/users/moritz/projects").then(resp => resp.json()).then(data => {
	let projects_div = document.getElementById('projects');
	console.table(data);
	for (const elm of data) {
    	    projects_div.innerHTML += `<div class="project_container"><a href="${elm.web_url}"><h2>${elm.name}</h2><p>${elm.description}</p></a></div>`;
	}
    });
}

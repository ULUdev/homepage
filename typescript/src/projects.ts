import {LitElement, html, css} from 'lit';
import {customElement, property, state, query} from 'lit/decorators.js';

// `hp` refers to homepage
@customElement('hp-projects')
export class Projects extends LitElement {

    static styles = css`
#projects {
    display: grid;
    grid-template-columns: auto auto auto;
    margin: auto;
width: 50%;
}
	.project_container {
	    text-align: center;
	    width: 50%;
	    margin: 20px 20px;
color: var(--fg);
background-color: var(--bg);
	    padding: 20px 20px;
	    border-radius: 10px;
	    transition: 600ms;
}
	    .project_container a {
		text-decoration: none;
color: var(--fg);
	    }
	    .project_container:hover {
		box-shadow: 10px 10px 10px black;
	    }
#nothing-found {
text-align: center;
color: gray;
}
@media only screen and (max-width: 90rem) {
#projects {
grid-template-columns: auto;
}
}
`;
    
    @state()
    private data = [];
    
    connectedCallback() {
	super.connectedCallback();
	this.loadData();
    }

    loadData() {
	fetch("https://gitlab.sokoll.com/api/v4/users/moritz/projects?pagination=keyset&per_page=100&order_by=id&sort=desc").then(resp => resp.json()).then(data => {
	    this.data = data;
	}).catch(err => console.error(err));
    }
    
    getProjects() {
        let error = false;
        if (!error && this.data.length != 0) {
            return html`<div id="projects" part="projects">${this.data.map((item) => html`<div class="project_container"><a href=${item.web_url}><h2>${item.name}</h2><p>${item.description}</p></a></div>`)}</div>`;
	    
        } else if (this.data.length == 0) {
	    return html`<div id="nothing-found" part="nothing-found"><p>Loading...</p></div>`;
	} else {
            return html`<div id="nothing-found" part="nothing-found"><p>No projects found</p></div>`;
        }
    }

    render() {
	return html`${this.getProjects()}`;
    }
}

import {LitElement, html} from 'lit';
import {customElement, property, state, query} from 'lit/decorators.js';

@customElement('projects-list')
export class Projects extends LitElement {

    @state()
    data = [];
    
    connectedCallback() {
	super.connectedCallback();
	this.loadData();
    }

    loadData() {
	fetch("https://gitlab.sokoll.com/api/v4/users/moritz/projects").then(resp => resp.json()).then(data => {
	    this.data = data;
	}).catch(err => console.error(err));
    }
    
    getProjects() {
        let error = false;
        if (!error && this.data) {
            return html`<div id="projects" part="projects">${this.data.map((item) => html`<div class="project_container"><a href=${item.web_url}><h2>${item.name}</h2><p>${item.description}</p></a></div>`)}</div>`;
	    
        } else if (!this.data) {
	    return html`<div id="nothing-found" part="nothing-found"><p>Loading...</p></div>`;
	} else {
            return html`<div id="nothing-found" part="nothing-found"><p>No projects found</p></div>`;
        }
    }

    render() {
	return html`${this.getProjects()}`;
    }
}

import {LitElement, html, css} from 'lit';
import {customElement, property, state, query} from 'lit/decorators.js';

@customElement('hp-navbar')
class Navbar extends LitElement {
    static styles = css`
:host {
color: var(--fg);
background-color: var(--bg);
margin: 0;
padding: 0;
position: fixed;
top: 0;
left: 0;
width: 100%;
box-shadow: 5px 5px 5px black;
}
ul {
color: var(--fg);
background-color: var(--bg);
list-style-type: none;
margin: 0;
padding: 0;
overflow: hidden;
}
li {
float: left;
text-transform: uppercase;
font-size: 20px;
}
li a {
display: block;
text-align: center;
padding: 10px 10px;
text-decoration: none;
color: var(--fg);
background-color: var(--bg);
transition: 300ms;
}
li a:hover {
background-color: var(--fg);
color: var(--bg);
}
`;
    
    navItem(name: string, url: string) {
	return html`<li><a href=${url}>${name}</a></li>`;
    }

    navItemRight(name: string, url: string) {
	return html`<li style="float: right"><a href=${url}>${name}</a></li>`;
    }
    
    render() {
	let nav_items = [];
	nav_items.push(this.navItem("home", "/"));
	nav_items.push(this.navItem("projects", "/projects"));
	nav_items.push(this.navItem("about", "/about"));
	nav_items.push(this.navItem("github", "http://github.com/ULUdev"));
	nav_items.push(this.navItem("gitlab", "http://gitlab.sokoll.com/moritz"));
	nav_items.push(this.navItemRight("login", "/login"));
	return html`<ul>${nav_items}</ul>`;
    }
}

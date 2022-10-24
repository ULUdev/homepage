import {LitElement, html, css} from 'lit';
import {property, state, query, customElement} from 'lit/decorators.js';

// TODO: functions for generating certain parts of the widgets

@customElement('hp-admin-panel')
class AdminPanel extends LitElement {
    static styles = css`
.ap-container {
display: grid;
margin: 60px;
}
.ap-widget {
color: var(--fg);
background-color: var(--bg);
border-radius: 10px;
text-align: center;
margin-bottom: 20px;
}
.ap-widget h2 {
text-transform: uppercase;
}
`;
    makeWidget(title: string, content) {
	return html`<div class="ap-widget"><h2>${title}</h2>${content}</div>`;
    }
    render() {
	let widgets = [];
	widgets.push(this.makeWidget("posts", "TODO"));
	widgets.push(this.makeWidget("users", "TODO"));
	return html`<div class="ap-container">${widgets}</div>`;
    }
}

import {litElement, html, css} from 'lit';
import {customElement, property, state, query} from 'lit/decorators.js';

@customElement('hp-cookiebanner')
class CookieBanner extends litElement {
    static styles = css`:banner {
        position: fixed;

    }`;
    @state
    clicked = localStorage.getItem('cookie_ok')==null?false:true;
    
    render() {
        if (!this.clicked) {
            return html`
            <div class="banner">
            <p>This page uses cookies to know if you are logged in or not</p>
            <button style="float: right" @click="${() => this.clicked = true}"
            </div>
            `;
        }
    }
}
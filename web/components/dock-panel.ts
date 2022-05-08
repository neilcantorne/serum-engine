
// @ts-ignore
import * as style from './dock-panel.scss'; style;

export class DockPanelElement extends HTMLElement {
    private shadow: ShadowRoot;

    constructor() {
        super();

        this.shadow = this.attachShadow({mode: 'closed'})
        
        Array.from(this.children).forEach((element) => {
            //todo: build
        })
    }

    static register() {
        customElements.define('dock-panel', DockPanelElement);
    }
}

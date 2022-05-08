import { DockButtonElement } from './dock-button';

export { DockButtonElement };

// @ts-ignore
import * as style from './dock-panel.scss'; style;

export class DockPanelElement extends HTMLElement {
    private shadow: ShadowRoot;

    constructor() {
        super();
        this.generateDOM();
    }

    private generateDOM() {
        this.shadow = this.attachShadow({mode: 'closed'})

        // Add item to DOM
        Array.from(this.children).forEach((node) => {
            if(node instanceof DockButtonElement) {
                this.shadow.appendChild(node.model);
            }
        })
        
        // Observe DOM changes
        let observer = new MutationObserver(this.handleDOMChanged.bind(this));
        observer.observe(this, { childList: true });
    }

    private handleDOMChanged(record: MutationRecord[]) {
        record.forEach(mutation => {
            // Add nodes to DOM
            mutation.addedNodes.forEach(node => {
                if(node instanceof DockButtonElement) {
                    this.shadow.appendChild(node.model);
                }
            })

            // Remove nodes from DOM
            mutation.removedNodes.forEach(node => {
                if(node instanceof DockButtonElement) {
                    this.shadow.removeChild(node.model)
                }
            })
        });
    }

    static register() {
        customElements.define('dock-button', DockButtonElement);
        customElements.define('dock-panel', DockPanelElement);
    }
}

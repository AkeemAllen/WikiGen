/**@type {import('./$types').PageLoad} */

import { wikis } from '../store';

export function load({params}) {
    wikis.set({
        "blaze-black": {
            name: "Blaze Black",
            description: "Wiki",
        },
        "volt-white": {
            name: "Volt White",
            description: "Wiki",
        }
    }) 
}
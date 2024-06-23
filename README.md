# WikiGen

WikiGen is tool that will allow Rom Hack creaters/maintainers (or anyone interested) to quickly and easily create user friendly wikis for their Pokemon Rom Hacks. The generated wikis will make it more convenient and pleasant for users to search through a Rom's documentation. No need for any more rich text files or word docs lol.

It allows you to document changes to:

- Pokemon (types, stats, move set, etc)
- Wild Encounters
- Trainer Encounters
- Moves
- ...and more to come

And have those changes immediately reflect in the wiki.

## Getting Started

### Prerequisites

1. Install [Python](https://www.python.org/downloads/) on your system and run `pip install mkdocs mkdocs-material` in a terminal.

2. Download and install the application binary for your operating system.

> [!IMPORTANT]  
> On Mac, when you try to open the app, you might get this popup message. "'WikiGen' cannot be opened because the developer cannot be verified."
> The app is currently unsigned. I plan on signing it in the future. For now, just follow the below instructions once to get past it.
> - Open Finder
> - Ctrl + click the app 
> - Click Open
> - You might get one more popup. Click open again.

## Creating and Running the Wiki Locally

**Creation**

1. Start the app and click the three dots to open the Wiki Select Menu and select "Create New Wiki".

2. Fill out the fields as instructed and click the "Create Wiki" button.

> [!TIP]  
> Use your github username to make deployment to github pages easier.
> However, you can use a different name if you prefer to host it elsewhere

3. Click the three dots again and select the new wiki. You'll see new pages for Pokemon, Routes, etc.

**Running the Server**

Navigate to the Wiki Testing page and follow the instructions to get the local server running.

## Modifying Pokemon

The app comes preloaded with data for all 1025 Pokemon. As of this release, you can modify a Pokemon's types, abilities, stats, evolution details, and movesets.

> [!NOTE]  
> The data is currently missing mega-evolutions, regional variants, and some minor forms.

On the Pokemon page, find, edit, and save any entry. Saved changes will be immediately reflected in the wiki. Give it a go!

## Generating Pokemon Pages

1. Open the "Generation" tab on the Pokemon page
2. Select a range (representing the Pokedex number for Pokemon) and click Generate.
3. Wiki pages for all Pokemon within that range will be generated.

## Modifying Routes

Open the Game Routes page and create a new route, eg, Nimbasa City, or Route 1. Open the newly created route by clicking its box. All changes to wild encounters and trainer encounters will be reflected in the wiki.

> [!TIP]  
> You'll also see an option to "Modify Encounter Types." Use this to add all the environments where a player can encounter wild pokemon on a route. Eg. Grass, Tall-Grass, Surfing, Fishing, Cave, etc.
>
> You can add as many as you wish.

**Documenting Wild Encounters**

1. Select the "Wild Encounters" tab. (You should be here by default)
2. Fill in the encounter type, Pokemon, and Encounter rate and click save.
3. The new route and wild encounter should be reflected in the wiki.
4. Do this for all wild pokemon on the route.

**Documenting Trainer Encounters**

There are quite a few things you can do when documenting trainer encounters.

##### Adding Pokemon

1. Select the "Trainer Encounters" tab
2. Add the Trainer Name, one of their Pokemon and its level and click save.
3. You should now see the Trainer and their Pokemon.
4. Add their other Pokemon, if they have more.

##### Deleting Pokemon

1. Hover over one of the Pokemon cards, you should see an edit and a delete icon appear.
2. Click the delete icon to delete a Pokemon from the team.
3. Deleing all Pokemon will delete the trainer as well.


##### Modifying Pokemon Moves, Held Items, etc

1. Hover over one of the Pokemon cards, you should see an edit and a delete icon appear.
2. Click the edit icon and a dialog will open.
3. If applicable, give the pokemon a nature, held item, ability, trainer version (will explain later), and moves.
4. Repeat for all pokemon on the team as necessary

##### Modifying Trainer Sprites

For some trainers, such as gym leaders, you may want a sprite.

1. Click the three horizontal dots beside the trainer name and select Add sprite.
2. Type in the name of a sprite, such as blue and click Set Sprite.

> [!NOTE]  
> All trainer sprites are taken from the [Pokemon Showdown Trainer Sprites](https://play.pokemonshowdown.com/sprites/trainers/) page.
> You can peruse this site if you're not sure what sprite name to use.
>
> Also, credit to all of the authors who made these sprites! Wouldn't be able to shout them all out here lol

##### Modifying Trainer Versions

It's possible for a trainer to have different versions depending on in-game factors. A good example would be your rival's starter, which will depend on the start your choose.

You can create different trainer versions to reflect their team based on in-game factors.

1. Click the three horizontal dots beside the trainer name and select Modify Trainer Version.
2. Type in a trainer version based on a defining charasteric, eg. "Fire" to represent their team if you pick the fire starter. and click "Create this option"
3. Click anywhere outside of the dialog to exit.
4. Hover over one of the Pokemon cards and click the edit icon.
5. Add the versions of the trainer that the pokemon will belong to.

## Generating Route Pages

## Deployment

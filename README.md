# WikiGen

WikiGen is tool that will allow Rom Hack creaters/maintainers (or anyone interested) to quickly and easily create user friendly wikis for their Pokemon Rom Hacks. The generated wikis will make it more convenient and pleasant for users to search through a Rom's documentation. No need for any more rich text files or word docs lol.

It allows you to document changes to:

- Pokemon (types, stats, move set, etc)
- Wild Encounters
- Trainer Encounters
- Moves
- ...and more to come

And have those changes immediately reflect in the wiki.

# Guide

This guide will take you from wiki creation to deployment. By the end, you'll have an example wiki that anyone can use.

## Getting Started

### Prerequisites

#### WikiGen Download and Installation

The first step is to install the binary for your operating system.

**Windows**

- Download and run the setup.exe from the releases
  - Since the app is unsigned you'll run into some warnings. Just choose to run anyway.

**Mac**

- Download and install the \*.dmg executable.

### **NOTE**

When you try to open the app, you might get this popup message from apple.

_Todo_: Add Image here.

You're getting this because the app is currently unsigned. I'd need apple developer account - costing $100 a year- to sign it (as far as I know). I'm not ready for that level of commitment yet so you can get past it with the below instructions if you choose.

- Open Finder
- Ctrl + click the app
- Click Open
- You might get one more popup, click open again.

You should only need to do this once.

**Linux**

- Download and depackage the \*.deb file.

_Note: I haven't gotten chance to test on a linux machine as yet so I'm not sure what complications you might face._

#### Python and Mkdocs

Next, We want to install Python and mkdocs. These are needed for building the actual wiki site.

1. Download and install [Python](https://www.python.org/downloads/) from their downloads page.
2. Open a terminal and run `pip install mkdocs mkdocs-material`.

Now that all the process stuff is out of the way, let's move on to the meat.

## Creating and Running the Wiki Locally

**Creation**

When you first open the app you'll see a blank page with a few elements. Click the three dots to open the Wiki Select Menu and click "Create New Wiki".

This will bring you to the create-wiki page. Fill out the fields as instructed (leaving Deployment URL blank for now) and click the Create Wiki button at the bottom.

_Note: For the author field, it asks for your github username so the wiki can be deployed to github pages. However, if you prefer to host it elsewhere, then you can use whatever name you'd like._

Once it's done, click the three dots again and select the wiki.

You should now see some new pages for Pokemon, Routes, etc.

**Running the Server**

Once you've created and selected you wiki, navigate to the Wiki Testing page. You can follow the instructions there to get a local server running. If you've already setup the Python and Mkdocs prerequisites, you can skip steps 1 and 2.

By the end, your wiki will be available in your browser for your own testing. I hope to automate this process in the future, but the manual steps should be easy to accomplish.

## Modifying Pokemon

The app comes preloaded with data for all 1025 Pokemon. This includes stats, types, movesets, etc. You can modify them as you see fit and a page will generate with the updates.

Simply navigate to the Pokemon page, search for a Pokemon, updates its information, and save the changes.

## Generating Pokemon Pages

### Type Change Page

### Evolution Change Page

## Modifying Routes

## Generating Route Pages

## Deployment

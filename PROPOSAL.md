# Marketplace Chat Room Proposal

## Team Members

Emmy Khawsam-ang : emmynk35
Alex Li : reallitho12
Athipat Pipatpinyopong: athipatpipat

## Plan

We plan to integrate a standard chat channel with an online marketplace to allow users to quickly and easily conduct trades while also keeping track of overall trends. The inspiration for this came from the online game Warframe. In this game, the in-game economy centers around trades between individual players which are facilitated enmasse through a single server-wide chat channel. This can make finding individual items for sale difficult, despite the chat filtering features, and also makes determining market conditions and optimal prices difficult as well, with many players relying on third-party websites to post their trades.

Our project proposal would be useful as our chat room would essentially function like the virtual equivalent of an exchange floor, where buy and sell orders can be typed into a channel that can be viewed as a "live-feed" while also being collected, aggregated, and displayed in a separate display that would show information like average, lowest, and highest price. This would allow people to find potential trade partners and spend more time actually conducting transactions.

**Difficulties:**
* Chat filtering features
* Making sure different parts of UI are synced
* Building GUI in a user friendly manner
* Handling simultaneous connections
* How people access the chat room

## Functional Requirements

**Must-haves:**
* Can enter buy and sell commmands w/ prices
* Must handle login, usernames (authentication service)
* Must be able to DM (direct message) people
* UI implementation
* Real time tracking of price changes

**Nice-to-haves:**
* Mock inventory of what you currently own

## Use Cases

Any environment where transactions are conducted on a direct peer-to-peer basis (the opposite example of this would be Runescape's Grand Exchange, where people simply enter in the buy/sell price and quantity of the item, and wait for the order to be filled. They cannot see who else is buying or selling, and the only information available to them is the "average" price of the item)

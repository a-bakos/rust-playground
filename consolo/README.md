# Solo card game

Solo is almost a clone of Uno and is a new variant of Mau-Mau with special action cards.

- Like Uno, Solo requires that a person play 1 card on their turn to the discard pile.

- The card they play must match the colour, value or symbol of the card on top of the discard pile.
- If a player cannot play a card they must draw a card to their hand.
- The aim of the game is to be the first player to ‘go out’
	- In doing so they will earn the total points of all the cards left in the other player’s hands.
	- The goal is to reach 600 points first.
- Unlike Uno, if a player is able to play the exact same card as the one on the top of the discard pile, they can do so out of turn
- The game then continues from that point in the circle of players and can result in 1 or more players being skipped in the play sequence.
- The deck consists of 112 cards in total.
- Each player gets eight cards.
- On your turn, you must play a card of the same color or of the same number as the last card played
	- alternatively, you can also play a black action card
- If you cannot or do not want to play, you must draw a card, that may be played immediately if it fits the play conditions.
- There are numbered cards ranging from 1-9 (2 of each) in 4 different colours – red, blue, green and yellow.
- There are ‘Take 2’ cards, which when played force the next player in turn to draw 2 cards.
	- A player can avoid drawing if they can play another ‘Take 2’ card immediately and force the pain (now 4 cards) onto the next person.
	- This can continue around the table until the next player cannot play a ‘Take 2’.
- The ‘Miss a Turn’ cards will jump the next person’s turn if the colour of the ‘Miss a Turn’ matches the colour of the top card on the discard pile.
- There are ‘Change Direction’ cards that reverse the order of play around the table as well.
- The ‘Wild’ cards featured in Uno are replaced with ‘Choose a Colour’ cards (with the same effect) and the ‘Draw 4’ cards of Uno are replaced with ‘Take 4 & Choose a Colour’ cards.
	- Again the person being asked to ‘Take 4’ can avoid this fate if they can play another ‘Take 4’ card immediately, forcing the next player to draw 8 unless they can play another card.
	- In Solo, there's so no bluffing mechanic.
- Solo adds 2 additional cards not seen in Uno
	- ‘Swap cards with Another Player’
	- ‘Change Cards all Round’.
	- The first is as simple as it sounds, whilst the latter forces all players to give their hand of cards to the next player, in the direction of play.

- Special action cards:
	- Make the next player miss their turn
	- Reverse the order of play
	- Make the next player draw two cards
	- Name a new colour to change the colour play condition
	- Name a new colour and make the next player picks up four cards!
	- Choose another player and swap cards with him or her
	- Exchange all player hands hands by moving them round one space in the play direction.

Number of players: 2 - 10
Total number of cards: 112

Cards:
- Yellow: [ 1-9 ] x 2, [ TAKE TWO ] x 2, [ MISS TURN ] x 2, [ SWAP ] x 1, [ REVERSE ] x 2
- Blue: [ 1-9 ] x 2, [ TAKE TWO ] x 2, [ MISS TURN ] x 2, [ SWAP ] x 1, [ REVERSE ] x 2
- Red: [ 1-9 ] x 2, [ TAKE TWO ] x 2, [ MISS TURN ] x 2, [ SWAP ] x 1, [ REVERSE ] x 2
- Green: [ 1-9 ] x 2, [ TAKE TWO ] x 2, [ MISS TURN ] x 2, [ SWAP ] x 1, [ REVERSE ] x 2
- Black: [ CHANGE CARDS ALL ROUND ] x 4, [ CHOOSE COLOUR ] x 4, [ TAKE FOUR ] x 4

---

Implementation details

Deck struct
	total,
	available,
	at_players,
	Methods:
		cards_left()
		shuffle()
Discard struct
	total,
	Methods:
		top_card()
		number_of_cards()
		reset()
Card struct
	color,
	value,
	face,
	Methods:
		get_color()
		get_value()
		get_face()
Player struct
	Methods:
		cards_in_hand()
		draw_card()
		discard()

import { CardView } from "../../bindings";
import { Card, CARD_ASPECT_RATIO } from "../../components/cards/Card";

type BattleDeckProps = {
  cards: CardView[];
};

export const DECK_CARD_WIDTH = 55;

export default function BattleDeck({ cards }: BattleDeckProps) {
  return (
    <div
      className="relative my-1"
      style={{
        width: `${DECK_CARD_WIDTH * CARD_ASPECT_RATIO}px`,
        height: `${DECK_CARD_WIDTH}px`,
      }}
    >
      {cards.map((card, index) => (
        <Card
          card={card}
          width={DECK_CARD_WIDTH}
          className="absolute origin-top-left -rotate-90"
          style={{
            top: `${DECK_CARD_WIDTH - getCardOffset(index)}px`,
            left: `${getCardOffset(index)}px`,
          }}
        />
      ))}
    </div>
  );
}

function getCardOffset(index: number) {
  if (index < 5) {
    return 0;
  } else if (index < 10) {
    return 1;
  } else if (index < 15) {
    return 2;
  } else if (index < 20) {
    return 3;
  } else if (index < 25) {
    return 4;
  } else {
    return 5;
  }
}

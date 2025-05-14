import { Card, CardProps } from '@/components/card';

interface CardGridProps {
  cards: CardProps[];
}

export const CardGrid = (props: CardGridProps) => {
  return (
    <div className='grid grid-cols-2 lg:grid-cols-3 gap-8 mx-auto'>
      {props.cards.map((card) => (
        <Card {...card.Icon} key={card.href} {...card} />
      ))}
    </div>
  );
};

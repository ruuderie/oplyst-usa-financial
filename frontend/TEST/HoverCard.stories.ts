import type { Meta, StoryObj } from '@storybook/vue3';

import HoverCard from '../components/ui/hover-card/HoverCard.vue';

const meta = {
  title: 'HoverCard',
  component: HoverCard,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof HoverCard>;

export default meta;
type Story = StoryObj<typeof HoverCard>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};
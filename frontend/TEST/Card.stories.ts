import type { Meta, StoryObj } from '@storybook/vue3';

import Card from '../components/ui/card/Card.vue';

const meta = {
  title: 'Card',
  component: Card,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof Card>;

export default meta;
type Story = StoryObj<typeof Card>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};
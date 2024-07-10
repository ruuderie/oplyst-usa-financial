import type { Meta, StoryObj } from '@storybook/vue3';

import CardFooter from '../components/ui/card/CardFooter.vue';

const meta = {
  title: 'CardFooter',
  component: CardFooter,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof CardFooter>;

export default meta;
type Story = StoryObj<typeof CardFooter>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};
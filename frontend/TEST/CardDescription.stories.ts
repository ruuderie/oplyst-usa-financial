import type { Meta, StoryObj } from '@storybook/vue3';

import CardDescription from '../components/ui/card/CardDescription.vue';

const meta = {
  title: 'CardDescription',
  component: CardDescription,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof CardDescription>;

export default meta;
type Story = StoryObj<typeof CardDescription>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};
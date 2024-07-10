import type { Meta, StoryObj } from '@storybook/vue3';

import CardTitle from '../components/ui/card/CardTitle.vue';

const meta = {
  title: 'CardTitle',
  component: CardTitle,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof CardTitle>;

export default meta;
type Story = StoryObj<typeof CardTitle>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};
import type { Meta, StoryObj } from '@storybook/vue3';

import CardHeader from '../components/ui/card/CardHeader.vue';

const meta = {
  title: 'CardHeader',
  component: CardHeader,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof CardHeader>;

export default meta;
type Story = StoryObj<typeof CardHeader>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};
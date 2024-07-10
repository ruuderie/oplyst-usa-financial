import type { Meta, StoryObj } from '@storybook/vue3';

import CardContent from '../components/ui/card/CardContent.vue';

const meta = {
  title: 'CardContent',
  component: CardContent,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof CardContent>;

export default meta;
type Story = StoryObj<typeof CardContent>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};
import type { Meta, StoryObj } from '@storybook/vue3';

import Button from '../components/ui/button/Button.vue';

const meta = {
  title: 'Button',
  component: Button,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof Button>;

export default meta;
type Story = StoryObj<typeof Button>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};
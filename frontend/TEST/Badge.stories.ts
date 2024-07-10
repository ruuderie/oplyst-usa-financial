import type { Meta, StoryObj } from '@storybook/vue3';

import Badge from '../components/ui/badge/Badge.vue';

const meta = {
  title: 'Badge',
  component: Badge,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof Badge>;

export default meta;
type Story = StoryObj<typeof Badge>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};
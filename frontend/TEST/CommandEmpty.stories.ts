import type { Meta, StoryObj } from '@storybook/vue3';

import CommandEmpty from '../components/ui/command/CommandEmpty.vue';

const meta = {
  title: 'CommandEmpty',
  component: CommandEmpty,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof CommandEmpty>;

export default meta;
type Story = StoryObj<typeof CommandEmpty>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};
import type { Meta, StoryObj } from '@storybook/vue3';

import Command from '../components/ui/command/Command.vue';

const meta = {
  title: 'Command',
  component: Command,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof Command>;

export default meta;
type Story = StoryObj<typeof Command>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};
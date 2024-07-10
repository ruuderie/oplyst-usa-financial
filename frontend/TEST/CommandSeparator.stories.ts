import type { Meta, StoryObj } from '@storybook/vue3';

import CommandSeparator from '../components/ui/command/CommandSeparator.vue';

const meta = {
  title: 'CommandSeparator',
  component: CommandSeparator,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof CommandSeparator>;

export default meta;
type Story = StoryObj<typeof CommandSeparator>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};
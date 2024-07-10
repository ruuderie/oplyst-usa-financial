import type { Meta, StoryObj } from '@storybook/vue3';

import CommandShortcut from '../components/ui/command/CommandShortcut.vue';

const meta = {
  title: 'CommandShortcut',
  component: CommandShortcut,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof CommandShortcut>;

export default meta;
type Story = StoryObj<typeof CommandShortcut>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};
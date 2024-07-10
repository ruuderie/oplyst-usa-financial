import type { Meta, StoryObj } from '@storybook/vue3';

import CommandDialog from '../components/ui/command/CommandDialog.vue';

const meta = {
  title: 'CommandDialog',
  component: CommandDialog,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof CommandDialog>;

export default meta;
type Story = StoryObj<typeof CommandDialog>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};
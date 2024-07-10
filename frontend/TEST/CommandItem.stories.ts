import type { Meta, StoryObj } from '@storybook/vue3';

import CommandItem from '../components/ui/command/CommandItem.vue';

const meta = {
  title: 'CommandItem',
  component: CommandItem,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof CommandItem>;

export default meta;
type Story = StoryObj<typeof CommandItem>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};
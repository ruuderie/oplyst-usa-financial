import type { Meta, StoryObj } from '@storybook/vue3';

import CommandGroup from '../components/ui/command/CommandGroup.vue';

const meta = {
  title: 'CommandGroup',
  component: CommandGroup,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof CommandGroup>;

export default meta;
type Story = StoryObj<typeof CommandGroup>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};
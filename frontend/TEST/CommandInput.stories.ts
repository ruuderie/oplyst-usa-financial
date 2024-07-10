import type { Meta, StoryObj } from '@storybook/vue3';

import CommandInput from '../components/ui/command/CommandInput.vue';

const meta = {
  title: 'CommandInput',
  component: CommandInput,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof CommandInput>;

export default meta;
type Story = StoryObj<typeof CommandInput>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};
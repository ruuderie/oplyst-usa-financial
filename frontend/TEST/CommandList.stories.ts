import type { Meta, StoryObj } from '@storybook/vue3';

import CommandList from '../components/ui/command/CommandList.vue';

const meta = {
  title: 'CommandList',
  component: CommandList,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof CommandList>;

export default meta;
type Story = StoryObj<typeof CommandList>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};
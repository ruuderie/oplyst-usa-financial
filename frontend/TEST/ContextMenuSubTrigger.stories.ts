import type { Meta, StoryObj } from '@storybook/vue3';

import ContextMenuSubTrigger from '../components/ui/context-menu/ContextMenuSubTrigger.vue';

const meta = {
  title: 'ContextMenuSubTrigger',
  component: ContextMenuSubTrigger,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof ContextMenuSubTrigger>;

export default meta;
type Story = StoryObj<typeof ContextMenuSubTrigger>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};
import type { Meta, StoryObj } from '@storybook/vue3';

import ContextMenuTrigger from '../components/ui/context-menu/ContextMenuTrigger.vue';

const meta = {
  title: 'ContextMenuTrigger',
  component: ContextMenuTrigger,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof ContextMenuTrigger>;

export default meta;
type Story = StoryObj<typeof ContextMenuTrigger>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};
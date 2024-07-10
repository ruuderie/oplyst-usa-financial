import type { Meta, StoryObj } from '@storybook/vue3';

import ContextMenuSeparator from '../components/ui/context-menu/ContextMenuSeparator.vue';

const meta = {
  title: 'ContextMenuSeparator',
  component: ContextMenuSeparator,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof ContextMenuSeparator>;

export default meta;
type Story = StoryObj<typeof ContextMenuSeparator>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};
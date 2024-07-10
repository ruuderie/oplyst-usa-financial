import type { Meta, StoryObj } from '@storybook/vue3';

import ContextMenuShortcut from '../components/ui/context-menu/ContextMenuShortcut.vue';

const meta = {
  title: 'ContextMenuShortcut',
  component: ContextMenuShortcut,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof ContextMenuShortcut>;

export default meta;
type Story = StoryObj<typeof ContextMenuShortcut>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};
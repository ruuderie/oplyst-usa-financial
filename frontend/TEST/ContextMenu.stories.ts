import type { Meta, StoryObj } from '@storybook/vue3';

import ContextMenu from '../components/ui/context-menu/ContextMenu.vue';

const meta = {
  title: 'ContextMenu',
  component: ContextMenu,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof ContextMenu>;

export default meta;
type Story = StoryObj<typeof ContextMenu>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};
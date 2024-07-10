import type { Meta, StoryObj } from '@storybook/vue3';

import ContextMenuSub from '../components/ui/context-menu/ContextMenuSub.vue';

const meta = {
  title: 'ContextMenuSub',
  component: ContextMenuSub,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof ContextMenuSub>;

export default meta;
type Story = StoryObj<typeof ContextMenuSub>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};
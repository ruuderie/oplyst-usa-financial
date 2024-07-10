import type { Meta, StoryObj } from '@storybook/vue3';

import ContextMenuRadioItem from '../components/ui/context-menu/ContextMenuRadioItem.vue';

const meta = {
  title: 'ContextMenuRadioItem',
  component: ContextMenuRadioItem,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof ContextMenuRadioItem>;

export default meta;
type Story = StoryObj<typeof ContextMenuRadioItem>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};